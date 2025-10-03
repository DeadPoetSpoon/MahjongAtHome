<template>
  <el-container class="main-layout">
    <el-header class="main-header">Header</el-header>
    <el-container class="main-inner-layout">
      <el-aside class="main-aside">
        <el-scrollbar>
          <el-menu default-active="1" router>
            <el-menu-item index="1" route="/">
              <el-icon><HomeFilled /></el-icon>
              <template #title>主页</template>
            </el-menu-item>
            <el-menu-item index="2" route="/settings">
              <el-icon><setting /></el-icon>
              <template #title>设置</template>
            </el-menu-item>
          </el-menu>
        </el-scrollbar>
      </el-aside>
      <el-main><slot /></el-main>
    </el-container>
  </el-container>
</template>

<script setup></script>
<style scoped>
.main-layout {
  display: flex;
  flex-direction: column;
  height: calc(100vh - var(--el-header-height));
}

.main-header {
  display: flex;
  text-align: center;
  align-items: center;
  justify-content: center;
}
.main-aside {
  max-width: 100px;
}
</style>
