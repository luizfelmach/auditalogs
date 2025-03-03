import http from 'k6/http';

 export const options = {
   duration: "30s",
   vus: 50
 };

export default function () {
  const url = 'http://localhost:8080';

  const payload = JSON.stringify({
    ip: "mock",
    mac: "mock",
    port: "mock"
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);
}