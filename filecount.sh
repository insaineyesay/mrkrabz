#!/bin/zsh
unset e f g c
typeset -A e=([js]="JavaScript" [ts]="TypeScript" [tsx]="TypeScript/React" [jsx]="JavaScript/React" [py]="Python" [java]="Java" [cs]="C#" [cpp]="C++" [c]="C" [h]="C/C++ Header" [php]="PHP" [rb]="Ruby" [go]="Go" [rs]="Rust" [swift]="Swift" [kt]="Kotlin" [scala]="Scala" [r]="R" [pl]="Perl" [sh]="Shell" [bash]="Bash" [sql]="SQL" [html]="HTML" [htm]="HTML" [css]="CSS" [scss]="SCSS" [sass]="Sass" [less]="Less" [json]="JSON" [xml]="XML" [yaml]="YAML" [yml]="YAML" [toml]="TOML" [md]="Markdown" [tex]="LaTeX" [vue]="Vue" [lua]="Lua" [dart]="Dart" [groovy]="Groovy" [m]="Objective-C" [mm]="Objective-C++" [clj]="Clojure" [ex]="Elixir" [erl]="Erlang" [hx]="Haxe" [zig]="Zig" [vb]="Visual Basic" [gradle]="Gradle" [tf]="Terraform")
typeset -A c=([js]=1 [ts]=1 [tsx]=1 [jsx]=1 [py]=1 [java]=1 [cs]=1 [cpp]=1 [c]=1 [h]=1 [php]=1 [rb]=1 [go]=1 [rs]=1 [swift]=1 [kt]=1 [scala]=1 [r]=1 [pl]=1 [sh]=1 [bash]=1 [sql]=1)
typeset -A f g
t=0
while IFS= read -r -d '' x; do
  d="${x##*.}"
  d="${(L)d}"
  [[ -n "${e[$d]}" ]] && {
    l="${e[$d]}"
    f["$l"]=$((${f["$l"]:-0}+1))
    t=$((t+1))
    [[ -n "${c[$d]}" ]] && g["$l"]=$((${g["$l"]:-0}+$(wc -l<"$x")))
  }
done < <(find . -type f -not -path '*/node_modules/*' -not -path '*/.git/*' -not -path '*/.next/*' -not -path '*/dist/*' -not -path '*/build/*' -not -path '*/target/*' -not -path '*/venv/*' -not -path '*/.venv/*' -not -path '*/env/*' -not -path '*/.idea/*' -not -path '*/vendor/*' -not -path '*/__pycache__/*' -print0)

echo "Code Files Count:"
echo ""

for l in "${(@k)f}"; do
  echo "$l|${f[$l]}|${g[$l]:-0}"
done | sort -t'|' -k2 -rn | while IFS='|' read -r l n o; do
  if [[ "$o" != "0" ]]; then
    echo "  $l: $n files | $o LOC"
  else
    echo "  $l: $n files"
  fi
done

echo ""
echo "Total: $t code files"