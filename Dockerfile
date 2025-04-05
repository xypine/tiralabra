FROM clux/muslrust:nightly AS rust
WORKDIR /app
RUN cargo install cargo-binstall
RUN cargo binstall wasm-pack
RUN cargo binstall wasm-bindgen-cli
COPY . .
RUN ./build_wasm.sh

FROM node:22-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
# Add this environment variable to make the app listen on all interfaces
ENV HOST=0.0.0.0
RUN corepack enable
ENV CI=1
COPY ./frontend/ /app
COPY --from=rust /app/frontend/pkg /app/pkg
COPY --from=rust /app/frontend/public /app/pkg
WORKDIR /app

FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

# FROM base
# COPY --from=prod-deps /app/node_modules /app/node_modules
# COPY --from=build /app/.output /app/.output
# EXPOSE 3000
# CMD [ "pnpm", "start" ]

FROM nginx
COPY --from=build /app/dist /usr/share/nginx/html
