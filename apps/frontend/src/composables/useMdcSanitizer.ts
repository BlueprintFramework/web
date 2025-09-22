import type { MDCElement, MDCNode, MDCRoot } from '@nuxtjs/mdc'

const ALLOWED_TAGS = new Set([
  'p',
  'h1',
  'h2',
  'h3',
  'h4',
  'h5',
  'h6',
  'ul',
  'ol',
  'li',
  'strong',
  'em',
  'code',
  'pre',
  'blockquote',
  'a',
  'br',
  'hr',
])

function sanitizeAstNode(node: MDCNode): MDCNode | null {
  if (node.type === 'text' || node.type === 'comment') {
    return node
  }

  const element = node as MDCElement
  if (!ALLOWED_TAGS.has(element.tag)) {
    return null
  }

  const sanitizedChildren =
    element.children
      ?.map(sanitizeAstNode)
      .filter((child): child is MDCNode => child !== null) || []

  return {
    ...element,
    children: sanitizedChildren,
  }
}

function sanitizeAst(root: MDCRoot): MDCRoot {
  return {
    type: 'root',
    children: root.children
      .map(sanitizeAstNode)
      .filter((child): child is MDCNode => child !== null),
  }
}

export const useMdcSanitizer = () => ({
  sanitizeAst,
  sanitizeAstNode,
})
