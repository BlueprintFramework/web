export default defineNuxtPlugin(() => {
  const visibility = useDocumentVisibility()

  //@ts-expect-error
  const isChrome = !!window.chrome && (!!window.chrome.webstore || !!window.chrome.runtime) //prettier-ignore

  const setSvgFavicon = (svgContent: string) => {
    const svgUrl = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svgContent)}`
    let link = document.querySelector<HTMLLinkElement>("link[rel~='icon']")

    if (!link) {
      link = document.createElement('link')
      link.rel = 'icon'
      document.head.appendChild(link)
    }
    link.href = svgUrl
  }

  const getFavicon = () => {
    return `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="#52A9FF" viewBox="0 0 3 3"><style>/*${Date.now()}*/@keyframes emblemSequence{0%,to{opacity:1}40%,60%{opacity:.2}}.cube{animation:emblemSequence .6s ease-in-out forwards;animation-delay:var(--delay);transition-duration:150ms;transition-property:opacity;transition-timing-function:cubic-bezier(.4,0,.2,1)}</style><path d="M1 0h1v1H1zM0 1h1v1H0z" class="cube" style="--delay:0ms"/><path d="M2 1h1v1H2zM1 2h1v1H1z" class="cube" style="--delay:70ms"/><path d="M2 2h1v1H2z" class="cube" style="--delay:140ms"/></svg>`
  }

  if (!isChrome) {
    watch(visibility, (current) => {
      if (current === 'visible') {
        setSvgFavicon(getFavicon())
      }
    })
  }

  setSvgFavicon(getFavicon())
})
