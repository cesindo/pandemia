const path = require('path')

process.env.VUE_APP_VERSION = require('./package.json').version;

module.exports = {
  publicPath: process.env.NODE_ENV === 'production'
    ? '/cc'
    : '/',
  productionSourceMap: false,
  pluginOptions: {
    'style-resources-loader': {
      patterns: [
        path.resolve(__dirname, 'src/assets/less/variable.less'),
        path.resolve(__dirname, 'src/assets/less/mixins.less'),
      ],
      preProcessor: 'less'
    }
  },
  runtimeCompiler: true,
  pwa: {
    themeColor: '#232731',    
    workboxOptions: {
      skipWaiting: true,
      clientsClaim: true,
    }  
  }
}

