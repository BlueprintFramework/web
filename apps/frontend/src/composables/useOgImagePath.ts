import { joinURL } from 'ufo'

export default (pagePath?: string) => {
  const appConfig = useAppConfig()
  const route = useRoute()
  return joinURL(
    appConfig.url,
    `/__og-image__/static`,
    pagePath || route.path,
    `og.png`
  )
}
