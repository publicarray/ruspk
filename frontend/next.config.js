
module.exports = {
    swcMinify: true,
    // experimental: {
    //     runtime: 'experimental-edge',
    // },
    poweredByHeader: false,
    images: {
        unoptimized: true,
        // loader: 'custom',
        domains: [
            'localhost',
            '127.0.0.1',
            '127.0.0.1:8080',
            'img.shields.io'
        ]
    }
}
