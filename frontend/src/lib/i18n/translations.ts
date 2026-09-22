// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

export const translations = {
	en: {
		meta: { label: 'English' },
		settings: {
			description: 'Adjust payload sizes used for bandwidth measurement.',
			download: 'Download Payload Size',
			downloadDescription: 'Amount of data transferred during the download phase.',
			page: 'Settings',
			save: 'Save Settings',
			saved: 'Saved settings',
			title: 'Test Configuration',
			upload: 'Upload Payload Size',
			uploadDescription: 'Amount of data transferred during the upload phase.'
		},
		stage: {
			download: 'Download',
			ping: 'Ping',
			upload: 'Upload'
		},
		test: {
			failed: 'Speed test failed',
			inProgressDownload: 'Downloading...',
			inProgressPing: 'Pinging...',
			inProgressUpload: 'Uploading...',
			start: 'Start'
		}
	},

	'pt-BR': {
		meta: { label: 'Português' },
		settings: {
			description: 'Ajuste os tamanhos de payload usados para medição de banda.',
			download: 'Tamanho do Payload de Download',
			downloadDescription: 'Quantidade de dados transferidos durante a fase de download.',
			page: 'Configurações',
			save: 'Salvar Configurações',
			saved: 'Configurações salvas',
			title: 'Configurações do Teste',
			upload: 'Tamanho do Payload de Upload',
			uploadDescription: 'Quantidade de dados transferidos durante a fase de upload.'
		},
		test: {
			failed: 'Teste de velocidade falhou',
			inProgressDownload: 'Baixando...',
			inProgressPing: 'Pingando...',
			inProgressUpload: 'Enviando...',
			start: 'Começar'
		}
	}
} as const;

type TranslatableStructure = Omit<typeof translations.en, 'meta'>;

type NestedKeys<T> = T extends object
	? {
			[K in keyof T & string]: T[K] extends object ? `${K}.${NestedKeys<T[K]>}` : K;
		}[keyof T & string]
	: never;

export type TranslationKey = NestedKeys<TranslatableStructure> | (string & {});
