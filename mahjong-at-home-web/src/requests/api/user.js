import { request } from '../request'

export function Signup(body) {
  return request({
    url: `/user/signup`,
    method: 'post',
    data: body,
  })
}

export function Login(body) {
  return request({
    url: `/user/login`,
    method: 'post',
    data: body,
  })
}
