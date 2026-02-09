import { describe, expect, it } from "vitest";
import type { NextResponse } from "next/server";
import { NextRequest } from "next/server";
import { proxy } from "../proxy";

const makeRequest = (path: string, token?: string) => {
  const headers: HeadersInit = token ? { cookie: `token=${token}` } : {};
  const request = new Request(`http://localhost${path}`, { headers });
  return new NextRequest(request);
};

const expectRedirect = (response: NextResponse, path: string) => {
  expect(response.headers.get("location")).toBe(`http://localhost${path}`);
};

describe("proxy route guard", () => {
  it("redirects / to /auth/login when unauthenticated", () => {
    const response = proxy(makeRequest("/"));
    expectRedirect(response, "/auth/login");
  });

  it("redirects / to /dashboard when authenticated", () => {
    const response = proxy(makeRequest("/", "token"));
    expectRedirect(response, "/dashboard");
  });

  it("redirects authenticated users away from public auth routes", () => {
    const response = proxy(makeRequest("/auth/login", "token"));
    expectRedirect(response, "/dashboard");
  });

  it("redirects unauthenticated users away from protected routes", () => {
    const response = proxy(makeRequest("/dashboard"));
    expectRedirect(response, "/auth/login");
  });

  it("allows authenticated users to access protected routes", () => {
    const response = proxy(makeRequest("/dashboard", "token"));
    expect(response.headers.get("x-middleware-next")).toBe("1");
  });
});
