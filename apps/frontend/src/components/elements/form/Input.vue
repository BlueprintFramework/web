<template>
  <div class="form-field" :class="fieldClasses">
    <label v-if="label" :for="fieldId" class="form-label" :class="labelClasses">
      {{ label }}
      <span
        v-if="required && requiredIcon"
        class="ml-1 text-red-400"
        aria-label="required"
      >
        *
      </span>
    </label>

    <div class="input-wrapper" :class="wrapperClasses">
      <template v-if="!isValidating">
        <Icon
          v-if="leadingIcon"
          :name="leadingIcon"
          :class="iconClasses"
          class="input-icon input-icon--leading scale-85"
        />
      </template>
      <template v-else>
        <div
          class="input-icon input-icon--leading scale-85"
          :class="iconClasses"
        >
          <div
            class="h-4 w-4 animate-spin rounded-full border-2 border-blue-400 border-t-transparent"
          ></div>
        </div>
      </template>

      <button
        v-if="trailingIcon"
        type="button"
        class="input-icon input-icon--trailing"
        :class="[
          iconClasses,
          {
            'cursor-pointer rounded-md hover:bg-gray-700/50':
              trailingIconClickable,
          },
        ]"
        @click="handleTrailingIconClick"
        :aria-label="trailingIconLabel"
      >
        <Icon :name="trailingIcon" class="h-5 w-5" />
      </button>

      <input
        :id="fieldId"
        ref="inputRef"
        :type="computedType"
        :name="name"
        :value="modelValue"
        :placeholder="computedPlaceholder"
        :disabled="disabled"
        :readonly="readonly"
        :required="required"
        :aria-required="required"
        :aria-invalid="hasError"
        :aria-describedby="describedByIds"
        :autocomplete="autocomplete"
        :class="inputClasses"
        @input="handleInput"
        @blur="handleBlur"
        @focus="handleFocus"
        @keydown="handleKeydown"
      />
    </div>

    <div
      v-if="description && !hasError"
      :id="`${fieldId}-description`"
      class="form-help-text"
    >
      {{ description }}
    </div>

    <div
      v-if="hasError"
      :id="`${fieldId}-error`"
      class="form-error-text"
      role="alert"
      aria-live="polite"
    >
      <Icon name="memory:close" class="mt-0.5 h-4 w-4 flex-shrink-0" />
      <span>{{ errorMessage }}</span>
    </div>

    <div
      v-if="showSuccess && !hasError"
      :id="`${fieldId}-success`"
      class="form-success-text"
    >
      <Icon name="memory:check" class="mt-0.5 h-4 w-4 flex-shrink-0" />
      <span>{{ successMessage || 'Input is valid' }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ValidationRule {
  validator: (value: string) => boolean | Promise<boolean>
  message: string
  trigger?: 'input' | 'blur' | 'submit'
}

interface FormInputProps {
  label?: string
  name?: string
  type?: 'text' | 'email' | 'password' | 'tel' | 'url' | 'search' | 'number'
  placeholder?: string
  description?: string
  required?: boolean
  disabled?: boolean
  readonly?: boolean
  autocomplete?:
    | 'on'
    | 'off'
    | 'address-line1'
    | 'address-line2'
    | 'address-line3'
    | 'address-level1'
    | 'address-level2'
    | 'address-level3'
    | 'address-level4'
    | 'street-address'
    | 'country'
    | 'country-name'
    | 'postal-code'
    | 'name'
    | 'additional-name'
    | 'family-name'
    | 'given-name'
    | 'honoric-prefix'
    | 'honoric-suffix'
    | 'nickname'
    | 'organization-title'
    | 'username'
    | 'new-password'
    | 'current-password'
    | 'bday'
    | 'bday-day'
    | 'bday-month'
    | 'bday-year'
    | 'sex'
    | 'one-time-code'
    | 'organization'
    | 'cc-name'
    | 'cc-given-name'
    | 'cc-additional-name'
    | 'cc-family-name'
    | 'cc-number'
    | 'cc-exp'
    | 'cc-exp-month'
    | 'cc-exp-year'
    | 'cc-csc'
    | 'cc-type'
    | 'transaction-currency'
    | 'transaction-amount'
    | 'language'
    | 'url'
    | 'email'
    | 'photo'
    | 'tel'
    | 'tel-country-code'
    | 'tel-national'
    | 'tel-area-code'
    | 'tel-local'
    | 'tel-local-prefix'
    | 'tel-local-suffix'
    | 'tel-extension'
    | 'impp'
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  variant?: 'default' | 'filled' | 'outlined'

  // Icons
  leadingIcon?: string
  trailingIcon?: string
  trailingIconClickable?: boolean
  trailingIconLabel?: string
  requiredIcon?: boolean

  // Validation
  rules?: ValidationRule[]
  validateOnInput?: boolean
  validateOnBlur?: boolean
  errorMessage?: string
  successMessage?: string
  showSuccess?: boolean

  // State
  loading?: boolean

  // Accessibility
  'aria-label'?: string
  'aria-describedby'?: string
}

const props = withDefaults(defineProps<FormInputProps>(), {
  type: 'text',
  size: 'md',
  variant: 'default',
  validateOnInput: false,
  validateOnBlur: true,
  trailingIconClickable: false,
  showSuccess: false,
  requiredIcon: true,
})

const modelValue = defineModel<string>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  blur: [event: FocusEvent]
  focus: [event: FocusEvent]
  validate: [isValid: boolean, errors: string[]]
  'trailing-icon-click': []
}>()

const inputRef = ref<HTMLInputElement>()
const fieldId = useId()
const isFocused = ref(false)
const isTouched = ref(false)
const isDirty = ref(false)
const isValidating = ref(false)
const validationErrors = ref<string[]>([])
const showPassword = ref(false)

const hasError = computed(() => {
  return (
    (props.errorMessage && props.errorMessage.length > 0) ||
    validationErrors.value.length > 0
  )
})

const errorMessage = computed(() => {
  return props.errorMessage || validationErrors.value[0] || ''
})

const computedType = computed(() => {
  if (props.type === 'password') {
    return showPassword.value ? 'text' : 'password'
  }
  return props.type
})

const computedPlaceholder = computed(() => {
  return (
    props.placeholder ||
    (props.label && !isFocused.value ? '' : props.placeholder)
  )
})

const describedByIds = computed(() => {
  const ids = []
  if (props.description && !hasError.value) ids.push(`${fieldId}-description`)
  if (hasError.value) ids.push(`${fieldId}-error`)
  if (props.showSuccess && !hasError.value) ids.push(`${fieldId}-success`)
  if (props['aria-describedby']) ids.push(props['aria-describedby'])
  return ids.join(' ')
})

const fieldClasses = computed(() => ({
  'form-field--focused': isFocused.value,
  'form-field--error': hasError.value,
  'form-field--success': props.showSuccess && !hasError.value,
  'form-field--disabled': props.disabled,
  [`form-field--${props.size}`]: true,
  [`form-field--${props.variant}`]: true,
}))

const inputClasses = computed(() => [
  'form-input',
  'w-full transition-all duration-300 ease-out',
  'text-gray-100 placeholder:text-gray-500',
  'border rounded-lg backdrop-blur-sm',

  // Size variants
  {
    'h-8 px-3 text-sm': props.size === 'xs',
    'h-9 px-3 text-sm': props.size === 'sm',
    'h-11 px-4 text-base': props.size === 'md',
    'h-12 px-5 text-lg': props.size === 'lg',
    'h-14 px-6 text-xl': props.size === 'xl',
  },

  // Variant styles
  {
    // Default variant
    'bg-gray-800/40 border-neutral-700':
      props.variant === 'default' && !hasError.value && !isFocused.value,
    'bg-gray-800/60 border-gray-500/60':
      props.variant === 'default' && !hasError.value && isFocused.value,

    // Filled variant
    'bg-gray-800/60 border-transparent':
      props.variant === 'filled' && !hasError.value,

    // Outlined variant
    'bg-neutral-950 border-neutral-700':
      props.variant === 'outlined' && !hasError.value,
  },

  // State styles
  {
    // Focus states
    'focus:outline-none focus:ring-4 focus:ring-blue-500/20 focus:border-blue-500/60':
      !hasError.value,
    'focus:ring-red-500/20 focus:border-red-500/60': hasError.value,

    // Error states
    'border-red-500/60 bg-red-900/10': hasError.value,

    // Success states
    'border-emerald-500/60 bg-emerald-900/10':
      props.showSuccess && !hasError.value,

    // Disabled states
    'opacity-60 cursor-not-allowed bg-gray-800/20 border-gray-700/30':
      props.disabled,

    // Icon padding
    'pl-11': props.leadingIcon || isValidating.value,
    'pr-11': props.trailingIcon,
  },
])

const labelClasses = computed(() => [
  'form-label',
  'block text-sm font-medium mb-2 transition-all duration-200',
  {
    'text-gray-300': !hasError.value && !isFocused.value,
    'text-blue-400': isFocused.value && !hasError.value,
    'text-red-400': hasError.value,
    'text-emerald-400': props.showSuccess && !hasError.value,
    'opacity-60': props.disabled,
  },
])

const wrapperClasses = computed(() => [
  'relative',
  {
    'opacity-60': props.disabled,
  },
])

const iconClasses = computed(() => [
  'text-gray-400',
  {
    'text-blue-400': isFocused.value && !hasError.value,
    'text-red-400': hasError.value,
    'text-emerald-400': props.showSuccess && !hasError.value,
  },
])

// Event handlers
const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
  isDirty.value = true

  if (props.validateOnInput && isTouched.value) {
    validateField()
  }
}

const handleBlur = (event: FocusEvent) => {
  isFocused.value = false
  isTouched.value = true
  emit('blur', event)

  if (props.validateOnBlur) {
    validateField()
  }
}

const handleFocus = (event: FocusEvent) => {
  isFocused.value = true
  emit('focus', event)

  if (hasError.value && isDirty.value) {
    validationErrors.value = []
  }
}

const handleKeydown = (event: KeyboardEvent) => {
  if (props.type === 'password' && event.key === 'Escape') {
    showPassword.value = false
  }
}

const handleTrailingIconClick = () => {
  if (props.type === 'password') {
    showPassword.value = !showPassword.value
  }
  emit('trailing-icon-click')
}

const validateField = async () => {
  if (!props.rules || props.rules.length === 0) return true

  isValidating.value = true
  validationErrors.value = []

  try {
    for (const rule of props.rules) {
      const isValid = await rule.validator(modelValue.value || '')
      if (!isValid) {
        validationErrors.value.push(rule.message)
        break
      }
    }

    const isValid = validationErrors.value.length === 0
    emit('validate', isValid, validationErrors.value)
    return isValid
  } finally {
    isValidating.value = false
  }
}

defineExpose({
  validate: validateField,
  focus: () => inputRef.value?.focus(),
  blur: () => inputRef.value?.blur(),
})

watch(
  () => props.errorMessage,
  (newError) => {
    if (newError) {
      validationErrors.value = []
    }
  }
)
</script>

<style scoped>
@reference "~/assets/css/main.css"

.form-input {
  font-size: max(16px, 1rem);
}

@media (max-width: 768px) {
  .form-input {
    min-height: 44px;
  }
}

.input-icon {
  @apply z-5 pointer-events-none absolute flex h-11 w-6 items-center justify-center;
}

.input-icon--leading {
  @apply left-3;
}

.input-icon--trailing {
  @apply right-3;
}

.form-help-text {
  @apply mt-2 text-sm text-gray-400;
}

.form-error-text {
  @apply mt-2 flex items-start gap-2 text-sm text-red-400;
}

.form-success-text {
  @apply mt-2 flex items-start gap-2 text-sm text-emerald-400;
}

@media (forced-colors: active) {
  .form-input {
    background-color: Field;
    color: FieldText;
    border-color: FieldText;
  }

  .form-input:focus {
    outline: 2px solid ButtonText;
    outline-offset: 2px;
  }
}

@media (prefers-reduced-motion: reduce) {
  .form-input,
  .form-label,
  .input-icon {
    transition: none;
  }
}
</style>
