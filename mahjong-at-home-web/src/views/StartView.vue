<template>
  <div v-if="!hasLogin" class="main-content">
    <el-form class="custom-form" :model="loginForm" label-width="auto">
      <el-form-item label="账户">
        <el-input class="custom-input" v-model="loginForm.email" clearable />
      </el-form-item>
      <el-form-item label="密码">
        <el-input
          class="custom-input"
          v-model="loginForm.psd"
          clearable
          type="password"
          show-password
        />
      </el-form-item>
      <el-form-item class="custom-button-item">
        <el-button type="primary" @click="onLogin">登录</el-button>
        <el-button type="primary" @click="onSignup">注册</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>
<script setup>
import { ref, reactive } from 'vue'
import { Signup, Login } from '../requests/api/user'
import { useUserStore } from '../stores/user.js'

const hasLogin = ref(false)
const loginForm = reactive({
  email: '',
  psd: '',
})

const onLogin = () => {
  Login(loginForm)
    .then((res) => {
      console.log('登录', res)
      if (res.code == 'Success') {
        let userStore = useUserStore()
        userStore.token = res.data.token
      } else {
        console.log('登录失败: ', res.message)
      }
    })
    .catch((err) => {
      console.error(err)
    })
}
const onSignup = () => {
  Signup(loginForm)
    .then((res) => {
      console.log('注册', res)
    })
    .catch((err) => {
      console.error(err)
    })
}
</script>
<style scoped>
.main-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}
.custom-input {
  width: 100%;
}
.custom-form {
  width: 100%;
  max-width: 400px;
}
.custom-button-item :deep(.el-form-item__content) {
  justify-content: flex-end;
}
</style>
