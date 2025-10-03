import axios from 'axios'

export const request = axios.create({
  baseURL: '/api',
  timeout: 500000,
  headers: {
    'Content-Type': 'application/json',
  },
})
request.interceptors.request.use(
  (config) => {
    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

request.interceptors.response.use(
  (response) => {
    if (response.status == 200) {
      return response.data
    } else {
      return null
    }
  },
  (error) => {
    return Promise.reject(error)
  },
)
export default request
