import axios from "axios";

// Main server api
export const apiClient = axios.create({
  baseURL: "http://localhost:5000",
});

// Appends the authorization to the reuest headers.
apiClient.interceptors.request.use((config) => {
    const id = localStorage.getItem("userId");
    const secret = localStorage.getItem("secret");
    if (id && secret) config.headers.Authorization = `${id}:${secret}`;
    return config;
});

export const cloudFunctions = axios.create({
  baseURL: "https://cloud-functions-91972588391.us-central1.run.app",
});
// HOW TO USE
// const response = await apiClient.get("/games", {request_body});
