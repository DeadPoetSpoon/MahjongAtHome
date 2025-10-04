<template>
  <el-container class="main-layout">
    <el-main>
      <el-page-header class="custom-header" @back="onBack">
        <template #icon>
          <el-icon><Menu /></el-icon>
        </template>
        <template #title><el-text>导航</el-text></template>
      </el-page-header>
      <el-drawer v-model="isShowMenu" :with-header="false" direction="ltr" size="40%">
        <el-scrollbar>
          <el-menu default-active="1" router>
            <el-menu-item index="1" route="/">
              <el-icon><HomeFilled /></el-icon>
              <template #title><el-text>主页</el-text></template>
            </el-menu-item>
            <el-menu-item index="2" route="/settings">
              <el-icon><setting /></el-icon>
              <template #title><el-text>设置</el-text></template>
            </el-menu-item>
          </el-menu>
        </el-scrollbar>
      </el-drawer>
      <slot />
    </el-main>
  </el-container>
</template>

<script setup>
import { ref } from 'vue'

const isShowMenu = ref(false)

const onBack = () => {
  isShowMenu.value = !isShowMenu.value
}
</script>
<style scoped>
.main-layout {
  display: flex;
  flex-direction: column;
  height: calc(100vh - var(--el-header-height));
}
:deep(.el-drawer__body) {
  padding: 0px;
  padding-top: 50px;
}
.custom-header {
  padding-bottom: 20px;
}
</style>
