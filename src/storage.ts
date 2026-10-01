export function readSetting(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

export function writeSetting(key: string, value: string): boolean {
  try {
    localStorage.setItem(key, value);
    return true;
  } catch {
    return false;
  }
}

export const draftKey = "boxmaker-draft-v5";

export function projectFileError(error: unknown): string {
  if (error instanceof SyntaxError)
    return "Le fichier n’est pas un projet JSON valide.";
  return error instanceof Error ? error.message : String(error);
}
