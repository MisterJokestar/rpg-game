import axios from "axios";

/**
 * Pre-configured Axios instance for all backend API calls.
 *
 * The base URL is read from the `VITE_SERVER_BASE_URL` environment variable at
 * build time, falling back to `http://localhost:5000` for local development.
 *
 * A request interceptor automatically appends the `Authorization` header using
 * the `userId` and `secret` values stored in `localStorage`, so authenticated
 * endpoints are called without any extra setup in page components.
 *
 * @example
 * const response = await apiClient.get("/games");
 * const response = await apiClient.post("/game/new", { player_id, character_id });
 */
export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_SERVER_BASE_URL ?? "http://localhost:5000",
});

/**
 * Request interceptor that injects the `Authorization` header.
 *
 * Reads `userId` and `secret` from `localStorage` (set after a successful
 * login or account creation). If both values are present the header is set to
 * `"<userId>:<secret>"`. If either value is missing the header is omitted and
 * the request proceeds unauthenticated.
 */
apiClient.interceptors.request.use((config) => {
    const id = localStorage.getItem("userId");
    const secret = localStorage.getItem("secret");
    if (id && secret) config.headers.Authorization = `${id}:${secret}`;
    return config;
});
