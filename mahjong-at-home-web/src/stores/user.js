import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUserStore = defineStore('user', () => {
  const id = ref(null)
  const email = ref(null)
  const token = ref(null)

  return { id, email, token }
})
