export interface Params {
  model: "press-slide" | "legacy";
  object: [number, number, number];
  objectWeight: number | null;
  padding: number;
  objectClearance: number;
  paddingWeight: number;
  wall: number;
  floor: number;
  clearance: number;
  printer: "p1s" | "k2";
  plateMargin: number;
  online: boolean;
  filamentPrice: number;
  measuredTotal: number | null;
}
export interface Part {
  id: string;
  name: string;
  mesh: { vertices: number[][]; triangles: number[][] };
  size: number[];
  assembledOffset: number[];
  volume: number;
  fits: boolean;
}
export interface Quote {
  service: string;
  category: string;
  cents: number;
  counterCents: number;
  tracking: boolean;
  delay: string;
}
export interface Design {
  model: "press-slide" | "legacy";
  referencePlasticWeight: number;
  mechanism: {
    tongueCenter: number;
    tongueHalfWidth: number;
    tongueRoot: number;
    hookY: number;
    releaseTravel: number;
    beamThickness: number;
    engagement: number;
    lidZ: number;
    beamLength: number;
    tipThickness: number;
    pressY: number;
    estimatedForce: number;
    forceRange: [number, number];
    strainPercent: number;
    stopStrainPercent: number;
    headroom: number;
    rearAllowance: number;
    deflectionProfile: [number, number][];
  } | null;
  outer: number[];
  inner: number[];
  objectOffset: number[];
  orientedObject: number[];
  objectSpace: number[];
  mechanismExpansion: number[];
  parts: Part[];
  plasticWeight: number;
  totalWeight: number | null;
  materialCost: number;
  quotes: Quote[];
  warnings: string[];
  tariffDate: string;
  tariffValidUntil: string;
  printerVolume: number[];
}
export const defaults: Params = {
  model: "press-slide",
  object: [100, 70, 30],
  objectWeight: 80,
  padding: 0,
  objectClearance: 0.3,
  paddingWeight: 0,
  wall: 1.2,
  floor: 0.8,
  clearance: 0.3,
  printer: "p1s",
  plateMargin: 5,
  online: true,
  filamentPrice: 25,
  measuredTotal: null,
};

export async function engine<T>(
  action: string,
  params: Params,
  extras: Record<string, string> = {},
): Promise<T> {
  const request = JSON.stringify({ action, params, ...extras });
  if ("__TAURI_INTERNALS__" in window) {
    const { invoke } = await import("@tauri-apps/api/core");
    return JSON.parse(await invoke<string>("engine", { request }));
  }
  const response = await fetch("/api/engine", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: request,
  });
  const data = await response.json();
  if (!response.ok) throw new Error(data.error || "Le moteur ne répond pas.");
  return data;
}

export async function download(
  data: BlobPart,
  filename: string,
  type = "application/octet-stream",
) {
  if ("__TAURI_INTERNALS__" in window) {
    const { invoke } = await import("@tauri-apps/api/core");
    const bytes = Array.from(
      new Uint8Array(await new Blob([data], { type }).arrayBuffer()),
    );
    return invoke<boolean>("save_file", { filename, bytes });
  }
  const url = URL.createObjectURL(new Blob([data], { type }));
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  setTimeout(() => URL.revokeObjectURL(url), 10_000);
  return true;
}
