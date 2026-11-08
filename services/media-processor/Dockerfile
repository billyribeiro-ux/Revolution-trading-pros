FROM node:26.13.0-bookworm-slim AS build

WORKDIR /app
RUN npm install --global pnpm@12.16.1

COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY imagify/package.json imagify/tsconfig.json imagify/tsconfig.build.json ./imagify/
COPY services/media-processor/package.json services/media-processor/tsconfig.json ./services/media-processor/
RUN pnpm install --frozen-lockfile --ignore-scripts --filter @revolution/media-processor...

COPY imagify/src ./imagify/src
COPY services/media-processor/src ./services/media-processor/src
RUN pnpm --filter @revolution/imagify build

FROM node:26.13.0-bookworm-slim AS runtime-deps

WORKDIR /app
RUN npm install --global pnpm@12.16.1
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY imagify/package.json ./imagify/
COPY services/media-processor/package.json ./services/media-processor/
RUN pnpm install --frozen-lockfile --ignore-scripts --prod --filter @revolution/media-processor...

FROM node:26.13.0-bookworm-slim

ENV NODE_ENV=production
WORKDIR /app
COPY --from=runtime-deps --chown=node:node /app/node_modules ./node_modules
COPY --from=runtime-deps --chown=node:node /app/imagify/node_modules ./imagify/node_modules
COPY --from=runtime-deps --chown=node:node /app/services/media-processor/node_modules ./services/media-processor/node_modules
COPY --from=build --chown=node:node /app/imagify/package.json ./imagify/
COPY --from=build --chown=node:node /app/imagify/dist ./imagify/dist
COPY --from=build --chown=node:node /app/services/media-processor/package.json ./services/media-processor/
COPY --from=build --chown=node:node /app/services/media-processor/src ./services/media-processor/src

USER node
EXPOSE 19010
CMD ["node", "/app/services/media-processor/src/server.ts"]
