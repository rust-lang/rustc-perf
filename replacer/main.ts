import {
  registerDynamicLanguage,
  parse,
  SgNode,
  NapiConfig,
} from "@ast-grep/napi";
import path from "node:path";
import * as child_process from "node:child_process";
import * as util from "node:util";
import {match, P} from "ts-pattern";
import * as cmd_ts from "cmd-ts";

const exec = util.promisify(child_process.exec);

// const rootDir = path.normalize("../");
const rootDir = process.env["DIRENV_DIR"]?.slice(1) || path.normalize("../");

const targetDir = path.join(rootDir, "site/frontend");

function replaceMatchedNode(node: SgNode): string {
  const srcText = node.text();

  const matchPar = /^par_?(\d)?$/i.exec(srcText);
  if (matchPar) {
    const digit = matchPar[1];
    if (digit != null) {
      return `threads_${digit}`;
    } else {
      return `threads`;
    }
  }

  const matchParallel = /^(P|p)arallel(s)?$/.exec(srcText);
  if (matchParallel) {
    const result = match([matchParallel[1], matchParallel[2]])
      .with(["P", "s"], () => {
        return "FrontendThreadsCounts";
      })
      .with(["p", "s"], () => {
        return "frontendThreadsCounts";
      })
      .with(["P", P._], () => {
        return "FrontendThreads";
      })
      .with([P._, P._], () => {
        return "frontendThreads";
      })
      .exhaustive();
    return result;
  }

  return srcText;
}

function performSubstitutionOfParallelOnTS(src: string): string {
  const root = parse("typescript", src).root();

  const generalMatch: NapiConfig = {
    rule: {
      any: [
        {kind: "property_identifier"},
        {kind: "identifier"},
        {kind: "string_fragment"},
      ],
      regex: String.raw`(?i)^par(\d|allel)?s?$`,
    },
  };

  const matches = root.findAll(generalMatch);
  const edits = matches.map((n) => n.replace(replaceMatchedNode(n)));
  const newSrc = root.commitEdits(edits);

  return newSrc;
}

function performSingleSubstitutionOnVueByInjectionIdx(
  vueText: string,
  injectionIdx: number,
): {
  result: string;
  changed: boolean;
} {
  const vueRoot = parse("vue", vueText).root();

  const tsSearchRule: NapiConfig = {
    rule: {
      any: [
        {
          all: [
            {
              pattern: {
                context: "<script $$$>$CONTENT</script>",
                selector: "script_element",
              },
            },
            {
              has: {kind: "raw_text"},
            },
          ],
        },
        {
          all: [
            {
              pattern: {
                context: "{{$CONTENT}}",
                selector: "interpolation",
              },
            },
            {
              has: {kind: "raw_text"},
            },
          ],
        },
      ],
    },
  };

  const scriptNodes = vueRoot.findAll(tsSearchRule);

  if (scriptNodes.length <= injectionIdx) {
    return {result: vueText, changed: false};
  }
  const tsScriptNode = scriptNodes[injectionIdx].getMatch("CONTENT")!;
  const {start, end} = tsScriptNode.range();

  const newScriptText = performSubstitutionOfParallelOnTS(tsScriptNode.text());
  const newVueText =
    vueText.slice(0, start.index + 1) +
    newScriptText +
    vueText.slice(end.index);
  return {result: newVueText, changed: true};
}

function performAllSubstitutionsOnVue(vueText: string): string {
  let newVueText = vueText;
  let changed = true;

  for (let injectionIdx = 0; changed; injectionIdx++) {
    const {result: newVueText_, changed: changed_} =
      performSingleSubstitutionOnVueByInjectionIdx(newVueText, injectionIdx);
    changed = changed_;
    newVueText = newVueText_;
  }
  return newVueText;
}

async function performAllSubstitutionsOnAnyFile(
  filename: string,
): Promise<string> {
  const fileContents = await Deno.readTextFile(filename);
  const ext = path.extname(filename);
  switch (ext) {
    case ".vue":
      return performAllSubstitutionsOnVue(fileContents);
    case ".ts":
      return performSubstitutionOfParallelOnTS(fileContents);
    default:
      throw new EvalError(`unknown extension: '${ext}'`);
  }
}

async function getAllFilesWithExtensionInIndex(): Promise<string[]> {
  const possibleFiles: string[] = await (async () => {
    const {stdout, stderr} = await exec(`git ls-files ${targetDir}`);
    if (stderr != "") {
      console.error(stderr);
    }
    return stdout.split("\n").filter((x) => x.trim().length != 0);
  })();
  const filesMatching = possibleFiles.filter((x) => {
    return [".ts", ".vue"].includes(path.extname(x));
  });

  return filesMatching;
}

function gitDiffContents(file: string, newContents: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = child_process.spawn("git", [
      "--no-pager",
      "diff",
      "--no-index",
      "--color=always",
      "--",
      file,
      "-",
    ]);

    let out = "";
    let err = "";

    child.stdout.setEncoding("utf8");
    child.stderr.setEncoding("utf8");

    child.stdout.on("data", (c) => (out += c));
    child.stderr.on("data", (c) => (err += c));
    child.on("error", reject);

    child.on("close", (code) => {
      if (code === 0 || code === 1) {
        console.log(out);
        resolve(out);
      } else reject(new Error(err || `git exited with ${code}`));
    });

    child.stdin.end(newContents);
  });
}

async function main(action: "diff" | "apply") {
  // const argv = yargs(hideBin(process.argv)).parse();
  registerDynamicLanguage({
    vue: {
      libraryPath: "/home/heinwol/temp/tree-sitter/vue.so",
      extensions: ["vue"],
      languageSymbol: "tree_sitter_vue",
    },
  });

  const filesMatching = await getAllFilesWithExtensionInIndex();

  for (const file of filesMatching) {
    const newContents = await performAllSubstitutionsOnAnyFile(file);
    switch (action) {
      case "diff":
        await gitDiffContents(file, newContents);
        break;
      case "apply":
        await Deno.writeTextFile(file, newContents);
        break;
    }
  }
}

async function doRun() {
  const app = cmd_ts.command({
    name: "substitute-with-ast-grep",
    args: {
      // someArg: cmd_ts.positional({
      //   type: cmd_ts.string,
      //   displayName: "some arg",
      // }),
      action: cmd_ts.positional({
        type: cmd_ts.oneOf(["diff", "apply"]),
        displayName: "action",
      }),
    },
    handler: async ({action}) => {
      // console.log({someArg});
      await main(action);
    },
  });

  await cmd_ts.run(app, process.argv.slice(2));
}

await doRun();
