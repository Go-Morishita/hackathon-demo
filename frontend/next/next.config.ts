import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  webpack(config) {
    config.watchOptions = {
      poll: 300,
      aggregateTimeout: 300,
    };
    return config;
  },
};

export default nextConfig;
