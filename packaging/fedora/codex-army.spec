Name:           codex-army
Version:        0.151.0
Release:        1%{?dist}
Summary:        Codex Army command-line coding agent
License:        Apache-2.0
URL:            https://github.com/sieciowiecxyz/codex-army
Source0:        codex
Source1:        codex-code-mode-host
Source2:        LICENSE
BuildArch:      x86_64
Conflicts:      codex

%description
Codex Army is a locally run command-line coding agent based on OpenAI Codex
with account-switch failover support.

%prep

%build

%install
install -D -m 0755 %{SOURCE0} %{buildroot}%{_bindir}/codex
install -D -m 0755 %{SOURCE1} %{buildroot}%{_bindir}/codex-code-mode-host
install -D -m 0644 %{SOURCE2} %{buildroot}%{_licensedir}/%{name}/LICENSE

%files
%license %{_licensedir}/%{name}/LICENSE
%{_bindir}/codex
%{_bindir}/codex-code-mode-host

%changelog
* Sun Aug 30 2026 Codex Army Maintainers <maintainers@sieciowiec.xyz> - 0.151.0-1
- Initial Codex Army package
