FROM node:slim as builder
WORKDIR /app
COPY ./ /app
ARG ARG_BACKEND_HOST
ENV BACKEND_HOST=$ARG_BACKEND_HOST
RUN npm run build

FROM robertohuertasm/microserver:latest
COPY --from=builder /app/dist /app/
