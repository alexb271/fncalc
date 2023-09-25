project('fncalc-gui', 'cpp', default_options : 'cpp_std=c++17', version: '1.0')

gtkdep = dependency('gtkmm-4.0', version: '>= 4.8')
subdir('fncalc-gui')

message('Building project \'fncalc\' with Cargo')
run_command('cargo', 'build', '--release', '--manifest-path',
  meson.current_source_dir() + '/Cargo.toml', check : true)

compiler = meson.get_compiler('cpp')
path = join_paths(meson.current_source_dir(), 'target/release')

fncalc = declare_dependency(
  dependencies : compiler.find_library('fncalc', dirs : [path]))

executable('fncalc', source_files, dependencies: [fncalc, gtkdep])

