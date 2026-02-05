import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

const publicRoutes = ["/auth/login", "/auth/register", "/"];

export function proxy(request: NextRequest) {
  const token = request.cookies.get("token")?.value;
  const { pathname } = request.nextUrl;

  // Check if the current route is public
  // We treat '/' as exact match, others as prefixes to allow sub-paths like /auth/login/forgot-password if needed
  const isPublicRoute = publicRoutes.some((route) =>
    route === "/" ? pathname === route : pathname.startsWith(route),
  );

  // If NOT authenticated and trying to access a PROTECTED route (not public)
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
