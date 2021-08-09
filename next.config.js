module.exports = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  webpack: function (config) {
    config.module.rules.push({ test: /\.md$/, use: "raw-loader" });
    return config;
  },

  async redirects() {
    return [
      /*
      {
        source: "/cryptoaliens",
        destination: "/2021/04/cryptoaliens",
        permanent: false,
      },
      */
      {
        source: "/day/:day",
        destination: "/shaderday/:day",
        permanent: true,
      },
    ];
  },
};
