# Install PNPM and node_modules
FROM node:current-alpine as build-stage
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

WORKDIR /app
COPY ./pnpm-lock.yaml ./package.json .
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

# Build frontend
COPY . .
RUN pnpm run build

# Serve frontend with nginx
FROM nginx:stable-alpine

COPY ./nginx.conf .
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]

# Notes on Dockerfile
# To build: `docker build -t frontend .`
# To run: `docker run -it --rm --init -p 8080:80 frontend`
