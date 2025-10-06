FROM node:current-alpine AS builder
WORKDIR /mahjong_at_home
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY mahjong_at_hone.conf /etc/nginx/conf.d/mahjong_at_hone.conf
COPY --from=builder /mahjong_at_home/dist /mahjong_at_home
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
