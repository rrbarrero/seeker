import { getBaseUrl } from "./api-config";

type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

type RequestOptions = {
  method: HttpMethod;
  token?: string;
  body?: unknown;
  headers?: Record<string, string>;
};

const buildHeaders = (options: RequestOptions): HeadersInit => {
  const headers: Record<string, string> = {
    ...(options.headers ?? {}),
  };

  if (options.token) {
    headers.Authorization = `Bearer ${options.token}`;
  }

  if (options.body !== undefined && !(options.body instanceof FormData)) {
    headers["Content-Type"] = "application/json";
  }

  return headers;
};

const request = async (path: string, options: RequestOptions): Promise<Response> => {
  const headers = buildHeaders(options);
  const body =
    options.body === undefined
      ? undefined
      : options.body instanceof FormData
        ? options.body
        : JSON.stringify(options.body);

  return fetch(`${getBaseUrl()}${path}`, {
    method: options.method,
    headers,
    body,
  });
};

const tryParseJson = async <T>(response: Response): Promise<T | null> => {
  try {
    return (await response.json()) as T;
  } catch {
    return null;
  }
};

export const requestJson = async <T>(
  path: string,
  options: RequestOptions,
): Promise<{ response: Response; data: T | null }> => {
  const response = await request(path, options);
  const data = await tryParseJson<T>(response);
  return { response, data };
};

export const requestEmpty = async (
  path: string,
  options: RequestOptions,
): Promise<{ response: Response }> => {
  const response = await request(path, options);
  return { response };
};
