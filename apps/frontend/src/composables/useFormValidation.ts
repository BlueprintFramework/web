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

    name: (
      message = 'Can only contain alphabetical characters, numbers and underscores'
    ): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        const nameRegex = /^[a-zA-Z0-9_]+$/
        return nameRegex.test(value)
      },
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

    exact: (expected: string, message?: string): ValidationRule => ({
      validator: (value: string) => expected == value,
      message: message || `Must match expected answer`,
      trigger: 'blur',
    }),

    minLength: (min: number, message?: string): ValidationRule => ({
      validator: (value: string) => value.length >= min,
      message: message || `Must be at least ${min} characters long`,
      trigger: 'blur',
    }),

    maxLength: (max: number, message?: string): ValidationRule => ({
      validator: (value: string) => value.length <= max,
      message: message || `Must be ${max} characters or less`,
      trigger: 'blur',
    }),

    password: (
      message = 'Password must contain at least 8 characters with uppercase, lowercase, number, and special character'
    ): ValidationRule => ({
      validator: (value: string) => {
        const passwordRegex = /^(?=\d*)(?=[a-z]*)(?=[A-Z]*)(?=[\W]*).{8,512}$/
        return passwordRegex.test(value)
      },
      message,
      trigger: 'blur',
    }),

    url: (message = 'Please enter a valid URL'): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        try {
          new URL(value)
          return true
        } catch {
          return false
        }
      },
      message,
      trigger: 'blur',
    }),

    code: (message?: string): ValidationRule => ({
      validator: (value: string) => {
        const codeRegex = /^[0-9]+$/
        if (value.length == 6 && codeRegex.test(value)) {
          return true
        } else if (value.length == 10) {
          return true
        }
        return false
      },
      message: message || 'Please enter a valid 2FA or recovery code',
      trigger: 'blur',
    }),

    pronouns: (
      message = 'Pronouns must follow a "foo/bar" or "foo/bar/fizz" structure'
    ): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        const pronounRegex = /^[a-zA-Z]+\/[a-zA-Z]+(?:\/[a-zA-Z]+)?$/
        return pronounRegex.test(value)
      },
      message,
      trigger: 'blur',
    }),

    extensionIdentifier: (
      message = 'Identifier must only contain lowercase a-z characters and be between 3 and 48 characters'
    ): ValidationRule => ({
      validator: (value: string) => {
        if (!value) return true
        const identifierRegex = /^[a-z]+$/
        if (
          !identifierRegex.test(value) ||
          value.length >= 48 ||
          value.length <= 3
        ) {
          return false
        }
        return true
      },
      message,
      trigger: 'blur',
    }),

    uniqueExtensionIdentifier: (
      current?: string,
      message?: string
    ): ValidationRule => ({
      validator: async (value: string) => {
        if (!value) return true
        if (current && current == value) return true
        if (!rules.extensionIdentifier(value)) return false

        const { error } = await useFetch(`/api/extensions/${value}`, {
          server: false,
        })

        if (error.value?.statusCode == 404) return true
        return false
      },
      message:
        message || 'Identifier is already taken by a published extension',
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
