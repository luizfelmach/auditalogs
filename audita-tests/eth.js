import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
   duration: "1m",
   vus: 1
};

const fixedHash = "0x97f978e380ff77d2be7f9735e353e1e417b00a080856fcffced3dd94bb1fa379";

export default function () {
  const randomIndex = Math.floor(Math.random() * 100000).toString()+ Math.floor(Math.random() * 100000).toString();;
  
  const payload = JSON.stringify({
    index: randomIndex,
    hash: fixedHash
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post('http://127.0.0.1:8080/eth', payload, params);

  sleep(1);
}
