module.exports = {
  getHasteName: (filePath) => {
    // Custom logic to generate unique haste names
    // You can modify this logic based on your project's structure
    return filePath.replace(/.*[\/\\]/, ''); // Returns only the file name
  },
};
