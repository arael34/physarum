## Physarum simulation written in Rust, using the Piston engine.

<div>
    <img src="images/physarum1.png" width="200px" />
    <img src="images/physarum2.png" width="200px" />
    <img src="images/physarum3.png" width="200px" />
    <img src="images/physarum4.png" width="200px" />
</div>

Complex, almost organic patterns can emerge from simple rules. Inspired by Sebastian Lague.

### Run for yourself:
Make sure you have cargo and git installed:
```bash
cargo version
git version
```
Next, clone from source.
```bash
git clone https://github.com/jonasiwnl/physarum
```
Create the `config.rs` file according to the example and tweak any parameters.
```bash
cp src/config.rs.example src/config.rs
```
Finally, run the --release version.
```bash
cargo run --release
```
