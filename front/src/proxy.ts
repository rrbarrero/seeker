import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

const publicRoutes = ["/auth/login", "/auth/register", "/auth/check-email", "/auth/verify-email"];

export function proxy(request: NextRequest) {
  const token = request.cookies.get("token")?.value;
  const { pathname } = request.nextUrl;

  // Handle root path redirect
  if (pathname === "/") {
    return NextResponse.redirect(new URL(token ? "/dashboard" : "/auth/login", request.url));
  }

  const isPublicRoute = publicRoutes.some((route) => pathname.startsWith(route));

  // If authenticated and trying to access auth routes, redirect to dashboard
  if (token && isPublicRoute) {
    return NextResponse.redirect(new URL("/dashboard", request.url));
  }

  // If NOT authenticated and trying to access a PROTECTED route
  if (!token && !isPublicRoute) {
    return NextResponse.redirect(new URL("/auth/login", request.url));
  }

  return NextResponse.next();
}

export const config = {
  // Match all request paths except for the ones starting with:
  // - api (API routes)
  // - _next/static (static files)
  // - _next/image (image optimization files)
  // - favicon.ico (favicon file)
  matcher: ["/((?!api|_next/static|_next/image|favicon.ico).*)"],
};
