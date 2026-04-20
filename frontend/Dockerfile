FROM node:22-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
ARG VITE_SERVER_BASE_URL=http://localhost:5000
ARG VITE_AUTH_BASE_URL=http://localhost:5000
ENV VITE_SERVER_BASE_URL=$VITE_SERVER_BASE_URL
ENV VITE_AUTH_BASE_URL=$VITE_AUTH_BASE_URL
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./
EXPOSE 3000
CMD ["node_modules/.bin/vike", "preview", "--host", "--port", "3000"]
