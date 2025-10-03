<template>
  <div v-if="!hasLogin" class="main-content">
    <el-form :model="loginForm" label-width="auto">
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
      </el-form-item>
    </el-form>
  </div>
</template>
<script setup>
import { ref, reactive } from 'vue'
import { Signup } from '../requests/api/user'

const hasLogin = ref(false)
const loginForm = reactive({
  email: '',
  psd: '',
})

const onLogin = () => {
  Signup(loginForm)
    .then((res) => {
      console.log('登录', res)
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
  width: 300px;
}
.custom-button-item {
  padding-left: 270px;
}
</style>
