module.exports = {
  async redirects() {
    return [
      {
        source: "/day/:day",
        destination: "/:day",
        permanent: true,
      },
    ];
  },
};
