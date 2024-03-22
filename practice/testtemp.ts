function main(lines: string[]) {
    /**
     * このコードは標準入力と標準出力を用いたサンプルコードです。
     * このコードは好きなように編集・削除してもらって構いません。
     *
     * This is a sample code to use stdin and stdout.
     * You can edit and even remove this code as you like.
    */
    lines.forEach((v, i) => console.log(`lines[${i}]: ${v}`));
  }
  function readFromStdin(): Promise<string[]> {
    return new Promise(resolve => {
      let data: string = "";
      process.stdin.resume();
      process.stdin.setEncoding("utf8");
      process.stdin.on("data", d => {
        data += d;
      });
      process.stdin.on("end", () => {
        resolve(data.split("\n"));
      });
    })
  }
  readFromStdin().then(main)