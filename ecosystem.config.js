module.exports = {
  apps: [
    {
      name: "echo",
      script: "./target/release/echo",
      exec_interpreter: "none",
      exec_mode: "fork_mode",
    },
  ],
};
