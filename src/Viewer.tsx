import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import type { Design, Params } from "./types";

interface Props {
  design: Design | null;
  params: Params;
  mode: "assembled" | "exploded" | "print";
  showObject: boolean;
  reset: number;
}
export function Viewer({ design, params, mode, showObject, reset }: Props) {
  const mount = useRef<HTMLElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
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
      const indexed = new THREE.BufferGeometry();
      indexed.setAttribute(
        "position",
        new THREE.Float32BufferAttribute(part.mesh.vertices.flat(), 3),
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
      const geometry = new THREE.BoxGeometry(...params.object);
      const material = new THREE.MeshStandardMaterial({
        color: 0xd1aa7c,
        transparent: true,
        opacity: mode === "assembled" ? 0.2 : 0.7,
        roughness: 0.8,
        depthWrite: false,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(
        ...(design.objectOffset.map((v, i) => v + params.object[i] / 2) as [
          number,
          number,
          number,
        ]),
      );
      model.add(mesh);
      objects.push(mesh);
    }
    const bounds = new THREE.Box3().setFromObject(model);
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
    fit();
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
  }, [design, params.object, mode, showObject, reset]);
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
