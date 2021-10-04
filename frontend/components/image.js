import NextImage from 'next/image'

const shimmer = (w, h) => `
<svg width="${w}" height="${h}" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
  <defs>
    <linearGradient id="g">
      <stop stop-color="#D1D5DB" offset="20%" />
      <stop stop-color="#6B7280" offset="50%" />
      <stop stop-color="#D1D5DB" offset="70%" />
    </linearGradient>
  </defs>
  <rect width="${w}" height="${h}" fill="#D1D5DB" />
  <rect id="r" width="${w}" height="${h}" fill="url(#g)" />
  <animate xlink:href="#r" attributeName="x" from="-${w}" to="${w}" dur="1s" repeatCount="indefinite"  />
</svg>`

const toBase64 = (str) =>
  typeof window === 'undefined'
    ? Buffer.from(str).toString('base64')
    : window.btoa(str)

const Image = ({src, className, alt = "", title = "", width, height, quality = 90, layout}) => (
    <NextImage
        className={className}
        alt={alt}
        title={title}
        src={src}
        placeholder="blur"
        blurDataURL={`data:image/svg+xml;base64,${toBase64(shimmer(width, height))}`}
        width={width}
        height={height}
        quality={quality}
        layout={layout}
    />
)

export default Image
