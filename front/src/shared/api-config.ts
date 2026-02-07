export const getBaseUrl = () => {
  if (typeof window === "undefined") {
    // Server-side: use the internal Docker network name
    // This assumes the backend service is named 'backend' in docker-compose.yml
    return process.env.INTERNAL_API_URL || "http://backend:3000";
  }
  // Client-side: use the public API URL
  return process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";
};
