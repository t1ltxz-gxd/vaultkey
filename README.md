<div align="center">

[![Preview](/assets/images/hero.png)](https://github.com/t1ltxz-gxd/vaultkey)
<p> 
    Rust library for generating secure and customizable passwords. With support for various character sets and lengths, 
    it helps you create strong and random passwords for any use case, ensuring high security for your applications.
</p>

---
[![GitHub License](https://img.shields.io/github/license/t1ltxz-gxd/vaultkey)](https://github.com/t1ltxz-gxd/vaultkey/blob/main/LICENSE "license")
[![Tests](https://img.shields.io/github/actions/workflow/status/t1ltxz-gxd/vaultkey/ci.yml?style=flat-square&logo=github&label=Tests)](https://github.com/t1ltxz-gxd/vaultkey/tests)
[![Forks](https://custom-icon-badges.demolab.com/github/forks/t1ltxz-gxd/vaultkey?logo=fork)](https://github.com/t1ltxz-gxd/vaultkey/network/members)
[![Contributors](https://custom-icon-badges.demolab.com/github/contributors/t1ltxz-gxd/vaultkey?logo=people)](https://github.com/t1ltxz-gxd/vaultkey/graphs/contributors)
[![Stars](https://custom-icon-badges.demolab.com/github/stars/t1ltxz-gxd/vaultkey?logo=star)](https://github.com/t1ltxz-gxd/vaultkey/stargazers)
[![Open issues](https://custom-icon-badges.demolab.com/github/issues-raw/t1ltxz-gxd/vaultkey?logo=issue)](https://github.com/t1ltxz-gxd/vaultkey/issues)


[![Powered by Rust](https://custom-icon-badges.herokuapp.com/badge/-Powered%20by%20Rust-0d1620?logo=rust)](https://www.rust-lang.org/ "Powered by Rust")
</div>

___

## 🧩 Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
vaultkey = {version = "*"}
```

## 📖 Usage

### Generate a random password
```rust
use vaultkey::PasswordBuilder;

fn main() {
    for i in 0..10 {
        let password = PasswordBuilder::default()
            .length(50)
            .with_uppercase(true)
            .min_digits(2)
            .min_specials(2)
            .avoid_ambiguous(true)
            .build()
            .unwrap();

        println!("Generated password {}: {}", i + 1, password);
    }
}
```

### Output:
```
Generated password 1: Y*4M>{e/>rDh:j=?K[]@9E#s|L&@k=BSpq9L+r@*RK_u>D;a-5
Generated password 2: 6.8CRjvc%4?JB[#$f3qmkX8dB@jC{|$-}4Ruh%+j|q)$raBv}Y
Generated password 3: qWTo%{^8,=bYQ]y3{3=xV869_Dy67%q7wbq!-xR.gv,eF]f>E}
Generated password 4: b?}..{=ued2axY@FHFcBr|]%{b{r*@2RErJKVK((.34v{(?e!2
Generated password 5: rF8n%ia%?8K4]52.gD86K,<$fH&%yg77;P?_#+G9JAtr::GWLp
Generated password 6: v#ZY>#tH[ufX+=4eYYX}$sQ<=3-8([j%f<dJdXadyc]3E,yB!8
Generated password 7: SWcAy%WsE#d]bZj2#!$5cffw&-@!9n_{83wEmu]/P:#>;wV_5j
Generated password 8: 63#wX=vVStBY?_:+S-|_mt6.u;/n+}|S+Qvetp95PCG|st&;2t
Generated password 9: YryFs}(!9Nq#?-$z!<huF8$8=ufg4m<QWZTaNMLr)n4>5sm/)]
Generated password 10: bM+HQ!t[s2b)8q%6R|&Sa-4m9/T_[=imjDEZr>a&&gnN_-iRqQ
```

### See all examples in the `examples` folder.

___

## 🤝 Contributing

Contributions are what make the open source community an amazing place to learn, be inspired, and create.
Any contributions you make are **greatly appreciated**.

1. [Fork the repository](https://github.com/t1ltxz-gxd/vaultkey/fork)
2. Clone your fork `git clone https://github.com/t1ltxz-gxd/vaultkey.git`
3. Create your feature branch `git checkout -b feat-smth-amazing`
4. Stage changes `git add .`
5. Commit your changes `git commit -m 'feat: add some amazing feature'`
   - Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages.
   - Use `fix`, `feat`, `docs`, `style`, `refactor`, `perf`, `test`, `chore` prefixes.
   - Use the present tense ("add feature" not "added feature").
   - Use the imperative mood ("move cursor to..." not "moves cursor to...").
   - Limit the first line to 72 characters or less.
   - Reference issues and pull requests liberally after the first line.
6. Push to the branch `git push origin feat-smth-amazing`
7. Submit a pull request

## ❤️ Credits

Released with ❤️ by [Tilt](https://github.com/t1ltxz-gxd).
