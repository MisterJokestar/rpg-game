import axios from "axios";

// Main server api
export const apiClient = axios.create({
  baseURL: import.meta.env.VITE_SERVER_BASE_URL ?? "http://localhost:5000",
});

// Appends the authorization to the reuest headers.
apiClient.interceptors.request.use((config) => {
    const auth = localStorage.getItem("auth");
    if (auth) config.headers.Authorization = auth;
    return config;
});

// Sepereate cloud function authorization system.
export const authClient = axios.create({
  baseURL: import.meta.env.VITE_AUTH_BASE_URL ?? "http://localhost:5000",
});

// HOW TO USE
// const response = await apiClient.get("/games", {request_body});
