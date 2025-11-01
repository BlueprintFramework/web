export const useTurnstileModal = () => {
  const isOpen = ref(false)
  const captchaValue = ref('')
  let resolver:
    | ((value: { captcha: string; confirmed: boolean }) => void)
    | null = null

  const show = async () => {
    isOpen.value = true
    captchaValue.value = ''

    return new Promise<{ captcha: string; confirmed: boolean }>((resolve) => {
      resolver = resolve
    })
  }

  const close = (result: { captcha: string; confirmed: boolean }) => {
    isOpen.value = false
    resolver?.(result)
    resolver = null
  }

  return {
    isOpen: readonly(isOpen),
    captchaValue,
    show,
    close,
  }
}
