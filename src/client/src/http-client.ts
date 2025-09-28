import type { ApiError } from './types';

export type RestApiResponse<T> = {
  data: T;
  page: number;
  total: number;
  totalPages: number;
}

export interface ClientConfig {
  baseUrl?: string;
  timeout?: number;
  headers?: Record<string, string>;
}

export class ApiClient {
  private timeout: number;
  private headers: Record<string, string>;

  protected baseUrl: string;

  constructor(config: ClientConfig = {}) {
    this.baseUrl = config.baseUrl || 'http://localhost:7878';
    this.timeout = config.timeout || 10000;
    this.headers = {
      'Content-Type': 'application/json',
      ...config.headers,
    };
  }

  protected async request<T>(path: string, options: RequestInit = {}): Promise<RestApiResponse<T>> {
    const url = `${this.baseUrl}${path}`;
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(url, {
        ...options,
        headers: {
          ...this.headers,
          ...options.headers,
        },
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        const errorData: ApiError = await response.json().catch(() => ({
          message: `HTTP ${response.status}: ${response.statusText}`,
        }));
        throw new ApiClientError(errorData.message, response.status, errorData.code as string);
      }

      const responseJson = response.json() as unknown as RestApiResponse<T>;

      return responseJson;
    } catch (error) {
      clearTimeout(timeoutId);

      if (error instanceof ApiClientError) {
        throw error;
      }

      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          throw new ApiClientError('Request timeout', 408);
        }
        throw new ApiClientError(error.message, 0);
      }

      throw new ApiClientError('Unknown error occurred', 0);
    }
  }

  protected buildQueryString(params: Record<string, any> = {}): string {
    const filteredParams = Object.entries(params)
      .filter(([, value]) => value != null)
      .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(value)}`);

    return filteredParams.length > 0 ? `?${filteredParams.join('&')}` : '';
  }
}

export class ApiClientError extends Error {
  public readonly statusCode: number;
  public readonly code?: string;

  constructor(message: string, statusCode: number, code?: string) {
    super(message);
    this.name = 'ApiClientError';
    this.statusCode = statusCode;
    this.code = code;
  }

  public isClientError(): boolean {
    return this.statusCode >= 400 && this.statusCode < 500;
  }

  public isServerError(): boolean {
    return this.statusCode >= 500 && this.statusCode < 600;
  }
}
