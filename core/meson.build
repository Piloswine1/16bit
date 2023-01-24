project(
    '16bit',
    'cpp',
    default_options : [
        'optimization=3',
        'cpp_std=c++2a',
        'warning_level=3',
        'werror=true',
        'prefer_static=true',
        'default_library=static'
    ])

main_incdir = include_directories('src')

src = files([
  'src/cpu/cpu.cpp',
  'src/memory/memory.cpp',
  'src/memory/writablememory.cpp',
  'src/memory/memorymapper.cpp',
  'src/devices/screendevice.cpp'
])

subproj_opts = 'default_library=static'

fmt = subproject('fmt', default_options: subproj_opts).get_variable('fmt_dep')
plog = subproject('sergiusthebest-plog', default_options: subproj_opts).get_variable('plog_dep')
argparse = subproject('argparse', default_options: subproj_opts).get_variable('argparse_dep')

deps = [fmt, plog, argparse]

_exe = executable('16bit',
  sources: ['src/main.cpp', src],
  include_directories: main_incdir,
  dependencies: deps)

#-- tests
tests_incdir = include_directories('tests/include')

memory_tests = executable('memory_tests',
  sources: ['tests/memory.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: deps)
test('memory tests', memory_tests)

writable_memory_tests = executable('writable_memory_tests',
  sources: ['tests/writablememory.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: deps)
test('writable memory tests', writable_memory_tests)

memorymapped_tests = executable('memorymapped_tests',
  sources: ['tests/memorymapped.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: deps)
test('memorymapped tests', memorymapped_tests)

cpu_tests = executable('cpu_tests',
  sources: ['tests/cpu.cpp', src],
  include_directories: [tests_incdir, main_incdir],
  dependencies: deps)
test('cpu tests', cpu_tests)
#---
