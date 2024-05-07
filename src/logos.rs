pub fn free_bsd() -> String {
    let x = r"
    ${c2}```                        ${c1}`
    ${c2}` `.....---...${c1}....--.```   -/
    ${c2}+o   .--`         ${c1}/y:`      +.
    ${c2} yo`:.            ${c1}:o      `+-
      ${c2}y/               ${c1}-/`   -o/
     ${c2}.-                  ${c1}::/sy+:.
     ${c2}/                     ${c1}`--  /
    ${c2}`:                          ${c1}:`
    ${c2}`:                          ${c1}:`
     ${c2}/                          ${c1}/
     ${c2}.-                        ${c1}-.
      ${c2}--                      ${c1}-.
       ${c2}`:`                  ${c1}`:`
         .--             `--.
            .---.....----.
    ";
    self::colorise(x)
}

pub fn ubuntu() -> String {
    let x = r"
${c1}                         ./+o+-
${c2}                 yyyyy- ${c1}-yyyyyy+
${c2}              ${c2}://+//////${c1}-yyyyyyo
${c3}          .++ ${c2}.:/++++++/-${c1}.+sss/`
${c3}        .:++o:  ${c2}/++++++++/:--:/-
${c3}       o:+o+:++.${c2}`..```.-/oo+++++/
${c3}      .:+o:+o/.${c2}          `+sssoo+/
${c2} .++/+:${c3}+oo+o:`${c2}             /sssooo.
${c2}/+++//+:${c3}`oo+o${c2}               /::--:.
${c2}+/+o+++${c3}`o++o${c1}               ++////.
${c2} .++.o+${c3}++oo+:`${c1}             /dddhhh.
${c3}      .+.o+oo:.${c1}          `oddhhhh+
${c3}       +.++o+o`${c1}`-````.:ohdhhhhh+
${c3}        `:o+++ ${c1}`ohhhhhhhhyo++os:
${c3}          .o:${c1}`.syhhhhhhh/${c3}.oo++o`
${c1}              /osyyyyyyo${c3}++ooo+++/
${c1}                  ````` ${c3}+oo+++o:
${c3}                         `oo++.
";
    self::colorise(x)
}

pub fn arch() -> String {
    let x = r"
    ${c1}                   -`
                      .o+`
                     `ooo/
                    `+oooo:
                   `+oooooo:
                   -+oooooo+:
                 `/:-:++oooo+:
                `/++++/+++++++:
               `/++++++++++++++:
              `/+++o${c2}oooooooo${c1}oooo/`
    ${c2}         ${c1}./${c2}ooosssso++osssssso${c1}+`
    ${c2}        .oossssso-````/ossssss+`
           -osssssso.      :ssssssso.
          :osssssss/        osssso+++.
         /ossssssss/        +ssssooo/-
       `/ossssso+/:-        -:/+osssso+-
      `+sso+:-`                 `.-/+oso:
     `++:.                           `-/+/
     .`                                 `/";
    self::colorise(x)
}

pub fn garuda() -> String {
    let x = r#"
${c3}
                     .%;888:8898898:
                   x;XxXB%89b8:b8%b88:
                .8Xxd                8X:.
              .8Xx;                    8x:.
            .tt8x          ${c6}.d${c3}            x88;
         .@8x8;          ${c6}.db:${c3}              xx@;
       ${c4},tSXX°          .bbbbbbbbbbbbbbbbbbbB8x@;
     .SXxx            bBBBBBBBBBBBBBBBBBBBbSBX8;
   ,888S                                     pd!
  8X88/                                       q
  GBB.
   ${c5}x%88        d888@8@X@X@X88X@@XX@@X@8@X.
     dxXd    dB8b8b8B8B08bB88b998888b88x.
      dxx8o                      .@@;.
        dx88                   .t@x.
          d:SS@8ba89aa67a853Sxxad.
            .d988999889889899dd.
"#;
    self::colorise(x)
}

fn colorise(x: &str) -> String {
    x.replace("${c1}", "\x1B[0m\x1B[31m")
        .replace("${c2}", "\x1B[0m\x1B[32m")
        .replace("${c3}", "\x1B[0m\x1B[33m")
        .replace("${c4}", "\x1B[0m\x1B[34m")
        .replace("${c5}", "\x1B[0m\x1B[35m")
        .replace("${c6}", "\x1B[0m\x1B[36m")
}