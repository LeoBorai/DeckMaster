export interface ApiError {
  code?: string | null;
  message: string;
}

export interface PaginatedResponse<T> {
  page: number;
  total: number;
  totalPages: number;
  data: T[];
}
