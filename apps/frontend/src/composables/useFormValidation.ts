export interface ValidationRule {
  validator: (value: string) => boolean | Promise<boolean>
  message: string
  trigger?: 'input' | 'blur' | 'submit'
}

export const useFormValidation = () => {
  const rules = {
    required: (message = 'This field is required'): ValidationRule => ({
      validator: (value: string) => value.trim().length > 0,
      message,
      trigger: 'blur',
    }),

    email: (
      message = 'Please enter a valid email address'
    ): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
        return emailRegex.test(value)
      },
      message,
      trigger: 'blur',
    }),

    minLength: (min: number, message?: string): ValidationRule => ({
      validator: (value: string) => value.length >= min,
      message: message || `Must be at least ${min} characters long`,
      trigger: 'blur',
    }),

    password: (
      message = 'Password must contain at least 8 characters with uppercase, lowercase, number, and special character'
    ): ValidationRule => ({
      validator: (value: string) => {
        const passwordRegex =
          /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$/
        return passwordRegex.test(value)
      },
      message,
      trigger: 'blur',
    }),

    phone: (message = 'Please enter a valid phone number'): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        const phoneRegex = /^\+?[\d\s\-\(\)]{10,}$/
        return phoneRegex.test(value)
      },
      message,
      trigger: 'blur',
    }),
  }

  const debounce = (func: Function, delay: number) => {
    let timeoutId: NodeJS.Timeout
    return (...args: any[]) => {
      clearTimeout(timeoutId)
      timeoutId = setTimeout(() => func(...args), delay)
    }
  }

  return {
    rules,
    debounce,
  }
}
