export const truncate = (text, length = 50, withDot = true) => {
  let rv = text
  if (text.length <= length) {
    return text
  } 

  rv = text.substring(0, length)

  if (withDot) {
    return rv + '...'
  }

  return rv
}

export const url = (url) => {
  try {
    return new URL(url)
  } catch(e) {
    return url
  } 
}

