FROM node:22-alpine AS base

# Install pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Install dependencies first (layer caching)
COPY package.json pnpm-lock.yaml* pnpm-workspace.yaml* ./
RUN pnpm install

# Copy the rest of the files
COPY . .

# Development stage
FROM base AS runner
EXPOSE 3000

ENV PORT 3000
ENV HOSTNAME "0.0.0.0"

CMD ["pnpm", "dev"]
