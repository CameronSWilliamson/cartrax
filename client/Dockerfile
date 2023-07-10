FROM node:18-alpine as build
WORKDIR /app
COPY . /app
ARG API_URL="http://localhost:5000"
RUN npm install
RUN VITE_API_URL=${API_URL} npm run build

FROM nginx:1-alpine
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
