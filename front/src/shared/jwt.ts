export type JwtClaims = {
  sub?: string;
  exp?: number;
  email?: string;
};

const decodeBase64Url = (value: string): string => {
  const base64 = value.replace(/-/g, "+").replace(/_/g, "/");
  const padded = base64.padEnd(base64.length + ((4 - (base64.length % 4)) % 4), "=");

  if (typeof window !== "undefined" && typeof window.atob === "function") {
    return window.atob(padded);
  }

  return Buffer.from(padded, "base64").toString("utf-8");
};

export const decodeJwt = (token: string): JwtClaims | null => {
  try {
    const [, payload] = token.split(".");
    if (!payload) {
      return null;
    }
    const json = decodeBase64Url(payload);
    return JSON.parse(json) as JwtClaims;
  } catch {
    return null;
  }
};

export const getUserIdFromToken = (token?: string | null): string | null => {
  if (!token) {
    return null;
  }
  const claims = decodeJwt(token);
  return claims?.sub ?? null;
};
