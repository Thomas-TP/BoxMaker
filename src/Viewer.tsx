import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { Design } from "./types";

interface Props {
  design: Design | null;
  mode: "assembled" | "exploded" | "print" | "open";
  opening: number;
  showObject: boolean;
  reset: number;
}
export function Viewer({ design, mode, showObject, reset, opening }: Props) {
  const mount = useRef<HTMLElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const viewRef = useRef<{
    design: Design;
    mode: Props["mode"];
    reset: number;
    position: THREE.Vector3;
    target: THREE.Vector3;
  } | null>(null);
  const [error, setError] = useState("");
  useEffect(
    () => () => {
      rendererRef.current?.forceContextLoss();
      rendererRef.current?.dispose();
      rendererRef.current?.domElement.remove();
      rendererRef.current = null;
    },
    [],
  );
  useEffect(() => {
    void reset; // Explicit user request to reconstruct and recenter the camera.
    if (!mount.current || !design) return;
    const host = mount.current;
    let renderer: THREE.WebGLRenderer;
    try {
      renderer =
        rendererRef.current ??
        new THREE.WebGLRenderer({ antialias: true, alpha: true });
      rendererRef.current = renderer;
    } catch {
      setError(
        "L’aperçu nécessite WebGL. Les calculs et exports restent disponibles.",
      );
      return;
    }
    setError("");
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFShadowMap;
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.setClearColor(0x000000, 0);
    host.appendChild(renderer.domElement);
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(36, 1, 0.1, 20000);
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.maxPolarAngle = Math.PI / 2 + 0.08;
    const objects: THREE.Object3D[] = [];
    scene.add(new THREE.HemisphereLight(0xffffff, 0x8e9c88, 3));
    const sun = new THREE.DirectionalLight(0xfffaf0, 4);
    sun.position.set(150, 350, 200);
    sun.castShadow = true;
    sun.shadow.mapSize.set(2048, 2048);
    scene.add(sun);
    const fill = new THREE.DirectionalLight(0xdce8f7, 2);
    fill.position.set(-200, 100, -100);
    scene.add(fill);
    const model = new THREE.Group();
    model.rotation.x = -Math.PI / 2;
    scene.add(model);
    let printX = 0;
    for (const [index, part] of design.parts.entries()) {
      const vertices = part.mesh.vertices.map((v) => [...v]);
      const mechanism = design.mechanism;
      if (part.id === "lid" && mode === "open" && mechanism) {
        const pressed =
          opening < 30
            ? Math.min(1, opening / 15)
            : Math.max(0, (40 - opening) / 10);
        for (const v of vertices) {
          const x = v[0] + part.assembledOffset[0];
          const y = v[1] + part.assembledOffset[1];
          if (
            Math.abs(x - mechanism.tongueCenter) <=
              mechanism.tongueHalfWidth + 0.001 &&
            y > mechanism.tongueRoot
          ) {
            const profile = mechanism.deflectionProfile;
            const position = Math.min(
              profile.length - 1,
              Math.max(
                0,
                ((y - mechanism.tongueRoot) / profile[profile.length - 1][0]) *
                  (profile.length - 1),
              ),
            );
            const index = Math.min(profile.length - 2, Math.floor(position));
            v[2] -=
              pressed *
              (profile[index][1] +
                (position - index) *
                  (profile[index + 1][1] - profile[index][1]));
          }
        }
      }
      const indexed = new THREE.BufferGeometry();
      indexed.setAttribute(
        "position",
        new THREE.Float32BufferAttribute(vertices.flat(), 3),
      );
      indexed.setIndex(part.mesh.triangles.flat());
      const geometry = indexed.toNonIndexed();
      indexed.dispose();
      geometry.computeVertexNormals();
      const material = new THREE.MeshStandardMaterial({
        color: [0x76946a, 0xaebd91, 0xe1a266][index],
        roughness: 0.78,
        metalness: 0.02,
        side: THREE.DoubleSide,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      if (mode === "print") {
        mesh.position.set(printX, 0, 0);
        printX += part.size[0] + 15;
      } else {
        mesh.position.fromArray(part.assembledOffset);
        if (part.id === "lid" && mode === "open")
          mesh.position.y -=
            opening < 30
              ? Math.max(0, (opening - 15) / 15) * 8
              : opening < 40
                ? 8
                : 8 + ((opening - 40) / 60) * (design.outer[1] + 2);
        if (part.id === "key") mesh.scale.z = -1;
        if (mode === "exploded") {
          if (part.id === "lid")
            mesh.position.z += Math.max(24, design.outer[2] * 0.65);
          if (part.id === "key")
            mesh.position.z += Math.max(50, design.outer[2] * 1.2);
        }
      }
      model.add(mesh);
      objects.push(mesh);
      const edges = new THREE.LineSegments(
        new THREE.EdgesGeometry(geometry, 30),
        new THREE.LineBasicMaterial({
          color: 0x45573d,
          transparent: true,
          opacity: 0.2,
        }),
      );
      mesh.add(edges);
      objects.push(edges);
    }
    if (showObject && mode !== "print") {
      const geometry = new THREE.BoxGeometry(...design.orientedObject);
      const material = new THREE.MeshStandardMaterial({
        color: 0xd1aa7c,
        transparent: true,
        opacity: mode === "assembled" ? 0.2 : 0.7,
        roughness: 0.8,
        depthWrite: false,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(
        ...(design.objectOffset.map(
          (v, i) => v + design.orientedObject[i] / 2,
        ) as [number, number, number]),
      );
      model.add(mesh);
      objects.push(mesh);
    }
    const bounds =
      mode === "open"
        ? new THREE.Box3(
            new THREE.Vector3(0, 0, -design.outer[1]),
            new THREE.Vector3(
              design.outer[0],
              design.outer[2],
              design.outer[1] + 10,
            ),
          )
        : new THREE.Box3().setFromObject(model);
    const center = bounds.getCenter(new THREE.Vector3());
    const size = bounds.getSize(new THREE.Vector3());
    const extent = Math.max(size.x, size.y, size.z, 70);
    model.position.sub(new THREE.Vector3(center.x, 0, center.z));
    const ground = new THREE.Mesh(
      new THREE.PlaneGeometry(extent * 6, extent * 6),
      new THREE.ShadowMaterial({ opacity: 0.13 }),
    );
    ground.rotation.x = -Math.PI / 2;
    ground.position.y = -0.2;
    ground.receiveShadow = true;
    scene.add(ground);
    objects.push(ground);
    const grid = new THREE.GridHelper(
      Math.ceil(extent / 10) * 30,
      30,
      0xc4cdbf,
      0xd8ded3,
    );
    grid.position.y = -0.3;
    scene.add(grid);
    objects.push(grid);
    sun.shadow.camera.left = -extent * 2;
    sun.shadow.camera.right = extent * 2;
    sun.shadow.camera.top = extent * 2;
    sun.shadow.camera.bottom = -extent * 2;
    sun.shadow.camera.far = extent * 15;
    sun.shadow.bias = -0.0005;
    sun.position.set(extent, extent * 3, extent * 2);
    const fit = () => {
      const distance = extent * (camera.aspect < 1.2 ? 2.9 : 2.2);
      camera.position.set(distance * 0.86, distance * 0.72, distance);
      controls.target.set(0, size.y * 0.32, 0);
      controls.update();
    };
    const resize = () => {
      const w = host.clientWidth,
        h = host.clientHeight;
      renderer.setSize(w, h);
      camera.aspect = w / Math.max(1, h);
      camera.updateProjectionMatrix();
    };
    resize();
    const saved = viewRef.current;
    if (
      saved?.design === design &&
      saved.mode === mode &&
      saved.reset === reset
    ) {
      camera.position.copy(saved.position);
      controls.target.copy(saved.target);
      controls.update();
    } else {
      fit();
    }
    const observer = new ResizeObserver(resize);
    observer.observe(host);
    let frame = 0;
    const animate = () => {
      frame = requestAnimationFrame(animate);
      controls.update();
      renderer.render(scene, camera);
    };
    animate();
    return () => {
      viewRef.current = {
        design,
        mode,
        reset,
        position: camera.position.clone(),
        target: controls.target.clone(),
      };
      cancelAnimationFrame(frame);
      observer.disconnect();
      controls.dispose();
      for (const object of objects) {
        if (
          object instanceof THREE.Mesh ||
          object instanceof THREE.LineSegments
        ) {
          object.geometry.dispose();
          const materials = Array.isArray(object.material)
            ? object.material
            : [object.material];
          for (const material of materials) material.dispose();
        }
      }
      sun.shadow.dispose();
    };
  }, [design, mode, showObject, reset, opening]);
  return (
    <section
      className="three-view"
      ref={mount}
      aria-label="Aperçu 3D interactif de la boîte"
    >
      {error && <p className="viewer-error">{error}</p>}
    </section>
  );
}
